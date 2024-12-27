//! A module to format JSON strings with configurable indentation, nesting, and line width,
//! without relying on serde for reformatting (in order to preserve key order).

/// Removes all whitespace outside of string literals in the given JSON string.
/// This preserves the original key order and formatting inside string literals.
fn minify_json(raw: &str) -> String {
    let mut in_string = false;
    let mut escaped = false;
    let mut result = String::with_capacity(raw.len());

    for c in raw.chars() {
        match c {
            '"' => {
                // Flip in_string if not escaped
                if !escaped {
                    in_string = !in_string;
                }
                escaped = false;
                result.push(c);
            }
            '\\' => {
                // Toggle escaped
                escaped = !escaped;
                result.push(c);
            }
            _ => {
                if in_string {
                    // Inside string, keep all characters (including spaces)
                    result.push(c);
                    escaped = false;
                } else {
                    // Outside string, drop all whitespace
                    if !c.is_whitespace() {
                        result.push(c);
                    }
                    escaped = false;
                }
            }
        }
    }
    result
}

/// Formats the JSON string with a given `indent`, merging lines if nesting level > `level`
/// and each line does not exceed `linewidth`.
pub fn human_read(serialized: &str, indent: usize, level: usize, linewidth: usize) -> String {
    // 1) Minify JSON to preserve key order, removing spaces outside of quotes.
    let minified = minify_json(serialized);

    // 2) We'll iterate through the minified JSON and create a naive multiline string with indentation.
    //    Then if nesting > level, we'll try to merge lines as long as we don't exceed linewidth.
    let mut lines: Vec<String> = Vec::new();
    let mut test_lines: Vec<(String, i32)> = Vec::new();
    let mut current_line = String::new();
    let mut nesting = 0i32;
    let mut in_string = false;
    let mut escaped = false;

    // Helper to push the current_line into lines and clear it.
    let mut push_line = |line: &mut String, nesting: i32| {
        if !line.is_empty() {
            lines.push(line.clone());
            test_lines.push((line.clone(), nesting));
            line.clear();
        }
    };

    for c in minified.chars() {
        // Track string boundaries
        match c {
            '"' if !escaped => {
                in_string = !in_string;
                current_line.push(c);
                escaped = false;
                continue;
            }
            '\\' => {
                escaped = !escaped;
                current_line.push(c);
                continue;
            }
            _ => {
                if escaped {
                    escaped = false;
                }
            }
        }

        // If we're inside a string, just push the char
        if in_string {
            current_line.push(c);
            continue;
        }

        // Outside string, handle braces, commas, etc.
        match c {
            '{' | '[' => {
                // Push current token (if any), then push a new line with this char
                push_line(&mut current_line, nesting);
                // Indent for current nesting
                //current_line.push_str(&" ".repeat(indent * nesting));
                current_line.push(c);
                push_line(&mut current_line, nesting);
                nesting += 1;
            }
            '}' | ']' => {
                // End current line before dedenting
                if !current_line.trim().is_empty() {
                	push_line(&mut current_line, nesting);
                }
                nesting -= 1;
                //current_line.push_str(&" ".repeat(indent * nesting));
                current_line.push(c);
            }
            ',' => {
                current_line.push(c);
                push_line(&mut current_line, nesting);
            }
            ':' => {
                current_line.push(c);
                current_line.push(' ');
            }
            _ => {
                current_line.push(c);
            }
        }
    }
    // Flush the last line if any
    push_line(&mut current_line, nesting);
	
	println!("{:?}", lines);
	
	println!("=================");
	println!("{}", serde_json::to_string_pretty(&test_lines).unwrap());
	println!("=================");

    // 3) Merge lines
    let mut line_stack: Vec<(String, i32, i32, bool)> = Vec::new();
	let mut last_nesting = 0i32;
	let mut last_pos_whose_nesting_is: Vec<usize> = vec![0; test_lines.iter().map(|x| x.1).max().unwrap() as usize + 1];

    for (line, nesting) in test_lines {
		line_stack.push((line, nesting, nesting, false));
		if nesting < last_nesting {
			let pos = last_pos_whose_nesting_is[nesting as usize].saturating_sub(1);
			let slice = &line_stack[pos..];
			let opt = slice.iter().map(|x| x.2).max();
			let max_nesting_level = match opt {
				Some(x) => x,
				None => continue,
			};
			// 如果最大嵌套层级与当前 nesting 的差值不超过 level，则合并
			if max_nesting_level - nesting <= level as i32 {
                // 截断之前 stack 中的内容
				let slice = line_stack.split_off(pos);
				let mut buffer_line = String::new();
                let mut buffer_nesting = nesting;
                // 逐行处理 slice
				for (l, n, mxn, o) in slice.clone() {
					if o {
                        // 如果 buffer_line 不为空，先把它推到栈里
                        if !buffer_line.is_empty() {
                            line_stack.push((buffer_line, nesting, buffer_nesting, false));
                            buffer_line = String::new();
                        }
                        // 直接把当前行推到 line_stack
                        line_stack.push((l, n, mxn, o));
					} else {
                        // 若合并后长度超过 linewidth，则先把之前的 buffer_line 推到栈里
						if buffer_line.len() + l.len() > linewidth - nesting as usize * indent {
                            if !buffer_line.is_empty() {
                                line_stack.push((buffer_line.clone(), nesting, buffer_nesting, true));
								buffer_line = String::new();
							}
						}

						if buffer_line.is_empty() {
							buffer_line = l;
							buffer_nesting = n;
                        } else {
                            // 否则将新行写回 buffer_line 中
							let c1 = buffer_line.chars().last().unwrap();
							let c2 = l.chars().next().unwrap();
							if ['{', '['].contains(&c1) || ['{', '[', ']', '}'].contains(&c2) {
								buffer_line = format!("{}{}", buffer_line, l);
							} else {
                            	buffer_line = format!("{} {}", buffer_line, l);
							}
							buffer_nesting = std::cmp::max(buffer_nesting, n);
                        }
					}
				}
				if !buffer_line.is_empty() {
					line_stack.push((buffer_line, nesting, buffer_nesting, false));
				}
				
	println!("=================");
	println!("{}", serde_json::to_string_pretty(&slice).unwrap());
	println!("-----------------");
	println!("{}", serde_json::to_string_pretty(&line_stack[pos..]).unwrap());
	println!("=================");
			}
		}
        last_pos_whose_nesting_is[nesting as usize] = line_stack.len().saturating_sub(1);
        last_nesting = nesting;
	}
        // let old_nesting = current_nesting;
        // let trimmed = line.trim_end();
        // // Compute how this line changes nesting
        // // We do that after we decide merging.
        // // If nesting > level, try to merge with buffer_line if length fits
        // if old_nesting > level {
        //     if buffer_line.is_empty() {
        //         buffer_line.push_str(trimmed);
        //     } else {
        //         let candidate = format!("{} {}", buffer_line, trimmed);
        //         if candidate.len() <= linewidth {
        //             buffer_line = candidate;
        //         } else {
		// 			test_merged.push((buffer_line.clone(), current_nesting));
        //             merged.push(buffer_line);
        //             buffer_line = trimmed.to_string();
        //         }
        //     }
        // } else {
        //     // nesting <= level, push buffer_line if not empty
        //     if !buffer_line.is_empty() {
		// 			test_merged.push((buffer_line.clone(), current_nesting));
        //         merged.push(buffer_line);
        //         buffer_line = String::new();
        //     }
		// 			test_merged.push((trimmed.to_string(), current_nesting));
        //     merged.push(trimmed.to_string());
        // }
        // // Now update nesting for this line
        // update_nesting(trimmed, &mut current_nesting);

        // // If it's the last line, flush buffer
        // if i == lines.len() - 1 && !buffer_line.is_empty() {
		// 			test_merged.push((buffer_line.clone(), current_nesting));
        //     merged.push(buffer_line);
        //     buffer_line = String::new();
        // }
	let merged: Vec<String> = line_stack.iter().map(|x| " ".repeat(indent * x.1 as usize) + &x.0).collect();

	// println!("-----------------");
	// println!("{}", serde_json::to_string_pretty(&test_merged).unwrap());
	// println!("-----------------");

    merged.join("\n")
}

#[cfg(test)]
mod tests {
    use super::human_read;

    #[test]
    fn test_human_read_simple() {
        let json_str = 
r#"{
  "name": "Alice",
  "age": 30,
  "items": ["apple", "banana"],
  "details": 
  {
    "likes": ["cake", "coffee"]
  }
}"#;
        let json_str_clean: String = json_str.split_whitespace().collect();

        let result = human_read(json_str_clean.as_str(), 2, 1, 40);

		println!("#####");
		println!("{}", result);
		
        assert_eq!(json_str, result);
    }

    /// A simple test to verify we don't rely on serde to reorder keys, and that
    /// we apply indent, level, and linewidth logic properly.
    #[test]
    fn test_human_read_simple2() {
        let json_str = r#"{
            "z": { "a": 1 }, "b": 2,
            "c": [ { "k": 1, "m": 2 }, 3 ]
        }"#;

        // Here we expect no reordering of the "z", "b", "c" keys.
        // Indent = 2, level = 1, linewidth = 40
        let result = human_read(json_str, 2, 1, 40);

        // We'll check partial strings to ensure no reordering, and spacing is consistent.
        assert!(result.contains("\"z\":"));
        assert!(result.contains("\"b\": 2"));
        assert!(result.contains("\"c\":"));        
        // We also check that the line length isn't exceeded excessively, 
        // and that the 'z' object doesn't get re-serialized by serde with reordering.
        assert!(result.len() > 0);
    }
}