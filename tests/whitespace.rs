extern crate comments;


#[cfg(test)]
mod whitespace_test {
    #[test]
    fn c_test() {
        let script = r#"#include <stdio>

int main() {
    // testing
    printf("Hello, world!");
}
"#;
        assert_eq!(comments::c::strip(script).unwrap().lines().collect::<Vec<&str>>().len(), script.lines().collect::<Vec<&str>>().len());
    }
}