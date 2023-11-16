new version:
  #!/usr/bin/env nu
  mkdir versions/v{{version}}
  '[workspace]
  members = ["examples/*"]
  resolver = "2"
  ' | save versions/v{{version}}/Cargo.toml
  mkdir versions/v{{version}}/examples

