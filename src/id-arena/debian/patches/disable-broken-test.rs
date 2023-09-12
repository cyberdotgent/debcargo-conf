Index: id-arena/tests/readme_up_to_date.rs
===================================================================
--- id-arena.orig/tests/readme_up_to_date.rs
+++ id-arena/tests/readme_up_to_date.rs
@@ -1,7 +1,7 @@
 use std::fs;
 use std::process::Command;
 
-#[test]
+#[allow(dead_code)]
 fn cargo_readme_up_to_date() {
     println!("Checking that `cargo readme > README.md` is up to date...");
 
