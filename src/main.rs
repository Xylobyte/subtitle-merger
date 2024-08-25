use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/// Simple tool to merge two WebVTT transcripts into one with speakers names
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The first transcript
    file1: String,

    /// The speaker name for the first transcript
    #[clap(long = "s1", default_value = None)]
    file1speaker: Option<String>,

    /// The second transcript
    file2: String,

    /// The speaker name for the second transcript
    #[arg(long = "s2", default_value = None)]
    file2speaker: Option<String>,

    /// The output file without extension
    output: String,

    /// Remove comment subtitles
    #[clap(long = "rm-comment-sub", default_value_t = false)]
    remove_comment_subtitles: bool,
}

fn get_clue_number(clue: &str) -> usize {
    clue.split(" --> ").into_iter().next().unwrap()
        .replace(":", "")
        .replace(".", "")
        .parse::<usize>()
        .unwrap()
}

fn main() -> Result<(), String> {
    let args = Cli::parse();

    let file1 = File::open(&args.file1).map_err(|_| "Could not open ".to_string() + &args.file1)?;
    let file2 = File::open(&args.file2).map_err(|_| "Could not open ".to_string() + &args.file2)?;

    let mut reader1 = BufReader::new(file1);
    let mut reader2 = BufReader::new(file2);

    let mut buff1 = String::new();
    let mut buff2 = String::new();

    for _ in 0..2 {
        reader1.read_line(&mut buff1).map_err(|_| "Could not read transcript files")?;
        reader2.read_line(&mut buff2).map_err(|_| "Could not read transcript files")?;
    }

    if buff1.trim() != "WEBVTT" || buff2.trim() != "WEBVTT" {
        return Err("Transcripts are not in WebVTT format".to_string());
    }

    let mut final_blocks: Vec<(String, String)> = Vec::new();

    for (i, reader) in [&mut reader1, &mut reader2].iter_mut().enumerate() {
        let mut line_buff = String::new();

        let mut in_clue: Option<String> = None;
        let mut transcript_text = String::new();

        let mut actual_index = 0;

        let name_tag = if let Some(name1) = if i == 0 { &args.file1speaker } else { &args.file2speaker } {
            format!("<v {}>", name1)
        } else {
            "".parse().unwrap()
        };

        while reader.read_line(&mut line_buff).map_err(|_| "Could not read transcript files")? != 0 {
            if line_buff.trim().is_empty() {
                let ttt = transcript_text.trim();
                if (ttt.starts_with("[") && ttt.ends_with("]")) || (ttt.starts_with("*") && ttt.ends_with("*"))
                    && ttt.len() > 2 && args.remove_comment_subtitles {
                    // Ignore
                } else if let Some(clue) = &in_clue {
                    let el = (clue.clone(), format!("{} {}", name_tag, transcript_text.trim()));
                    if i == 0 {
                        final_blocks.push(el);
                    } else {
                        let actual_clue = get_clue_number(&clue);
                        loop {
                            let base_clue = get_clue_number(&final_blocks[actual_index].0);

                            if actual_clue <= base_clue {
                                final_blocks.insert(actual_index, el);
                                break;
                            } else if actual_index == final_blocks.len() - 1 {
                                final_blocks.push(el);
                                break;
                            } else {
                                actual_index += 1;
                            }
                        }
                    }
                }

                in_clue = None;
                transcript_text.clear();
            } else if line_buff.contains("-->") {
                in_clue = Some(line_buff.trim().to_string());
            } else if in_clue.is_some() {
                transcript_text.push_str(&line_buff);
            }

            line_buff.clear();
        }

        if let Some(clue) = &in_clue {
            final_blocks.push((clue.clone(), format!("{} {}", name_tag, transcript_text.trim())));
        }
    }

    let mut output_file = File::create(format!("{}.vtt", &args.output)).map_err(|_| format!("Could not create file {}.vtt", &args.output))?;
    let mut final_buff = String::new();

    println!();
    for (clue, transcript) in final_blocks {
        final_buff.push_str(format!("{}\n{}\n\n", clue, transcript).as_str());
    }

    println!("{}", final_buff);
    output_file.write_all(format!("WEBVTT\n\n{}", final_buff).as_ref()).map_err(|_| "Could not write in the file")?;

    Ok(())
}
