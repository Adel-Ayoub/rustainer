use std::os::raw::c_int;
use rand::Rng;

/// Signal sent when child process terminates
pub const SIGCHLD: c_int = 17;

/// Generate a random container ID
pub fn generate_container_id() -> String {
    let mut rng = rand::thread_rng();
    let id: String = (0..12)
        .map(|_| {
            let idx = rng.gen_range(0..16);
            "0123456789abcdef".chars().nth(idx).unwrap()
        })
        .collect();
    id
}

/// Parse image reference into (name, tag)
pub fn parse_image_ref(image_ref: &str) -> (&str, &str) {
    if let Some((name, tag)) = image_ref.split_once(':') {
        (name, tag)
    } else {
        (image_ref, "latest")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_container_id() {
        let id = generate_container_id();
        assert_eq!(id.len(), 12);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_parse_image_ref_with_tag() {
        let (name, tag) = parse_image_ref("alpine:3.18");
        assert_eq!(name, "alpine");
        assert_eq!(tag, "3.18");
    }

    #[test]
    fn test_parse_image_ref_without_tag() {
        let (name, tag) = parse_image_ref("alpine");
        assert_eq!(name, "alpine");
        assert_eq!(tag, "latest");
    }
}
