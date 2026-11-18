// Santxzz Optimizer - Core runtime

pub fn block_break_message(pos: [i32; 3]) -> String {
    format!(
        "[Santxzz Optimizer] Block position: {}, {}, {}",
        pos[0], pos[1], pos[2]
    )
}

pub fn before_break(pos: [i32; 3]) {
    eprintln!("{}", block_break_message(pos));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_preserves_signed_block_coordinates() {
        assert_eq!(
            block_break_message([-3, 64, 12]),
            "[Santxzz Optimizer] Block position: -3, 64, 12"
        );
    }
}
