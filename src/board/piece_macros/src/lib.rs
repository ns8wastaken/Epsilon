use proc_macro::TokenStream;

#[proc_macro]
pub fn define_pieces(input: TokenStream) -> TokenStream {
    let string = input.to_string();
    let trimmed = string.trim();

    let mut chars = trimmed.chars().peekable();
    let mut derives = String::new();

    if chars.peek() == Some(&'#') {
        // Parse derives
        while let Some(c) = chars.next() {
            derives.push(c);
            if c == ']' {
                break;
            }
        }

        // Remove whitespace following the derives
        while let Some(c) = chars.peek() {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }
    }

    let mut output = format!("{} pub enum PieceType {{", derives);

    // Parse pieces
    let remaining = chars.collect::<String>();
    let pieces: Vec<&str> = remaining
        .split(',')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .collect();

    for &piece in &pieces {
        output.push_str(&format!("{piece},"));
    }

    output.push('}');
    output.push_str(&format!("pub const PIECETYPE_COUNT: usize = {};", pieces.len()));

    // output.push_str(&format!("{} pub enum Piece {{", derives));
    // for (i, &piece) in pieces.iter().enumerate() {
    //     output.push_str(&format!("White{} = {},", piece, i + 1));
    //     output.push_str(&format!("Black{} = {},", piece, i + 9));
    // }
    // output.push('}');

    output.parse().unwrap()
}
