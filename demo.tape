Output assets/demo.gif

Set Shell "bash"
Set FontSize 18
Set Width 900
Set Height 560
Set Padding 24
Set Theme "Builtin Solarized Dark"
Set TypingSpeed 70ms

Type 'exec bash --noprofile --norc -i'
Enter
Sleep 600ms
Type 'export PATH="$PWD/target/debug:$PWD/demo/bin:$PATH"'
Enter
Sleep 400ms
Type 'eval "$(pls init bash)"'
Enter
Sleep 500ms
Type 'protected-touch /tmp/pls-demo-secret.txt'
Enter
Sleep 1400ms
Type 'pls'
Enter
Sleep 1600ms
Type 'ls -l /tmp/pls-demo-secret.txt'
Enter
Sleep 2200ms
