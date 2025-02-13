zwjstego is a rust utility to encode and decode zwj messages into a unicode character.

Usage
-----

`zwjstego [-d] [-e <base unicode character>] <text to encode or decode>`

 - `<text>` : text to encode or decode. Can also be passed via stdin.
 - -e : specify base unicode character to embed the message in
 - -d : switch to decode mode

<img width="746" alt="image" src="https://github.com/user-attachments/assets/9837ba53-e87f-46fb-9cfa-b1d0573ace76" />

Building
--------

Uses rust's cargo system.

`cargo build -r`
Will build the release binary, and place it in `target/release/zwjstego`
