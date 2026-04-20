Output assets/demo.gif

Set Shell "bash"
Set FontSize 18
Set Width 900
Set Height 560
Set Padding 24
Set Theme "Builtin Solarized Dark"
Set TypingSpeed 35ms

Type 'export PATH="$PWD/target/debug:$PWD/demo/bin:$PATH"'
Enter
Type 'eval "$(pls init bash)"'
Enter
Type 'protected-touch ./secrets.txt'
Enter
Sleep 800ms
Type 'pls'
Enter
Sleep 1200ms
Type 'ls -l ./secrets.txt'
Enter
Sleep 1800ms
