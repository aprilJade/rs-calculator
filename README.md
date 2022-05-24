# re-calculator
This is minimal calculator implemented with rust programming language. 
![img](/resource/rs-calculator.png)
## How to build
0. This source need to cargo and gtk4 library. If they aren't installed, you should install on your environment.
1. Clone, build, run this repository
```bash
git clone https://github.com/aprilJade/rs-calculator.git
cd rs-calculator
cargo run
```
## How to calculate
1. Get infix expression from user.
2. Convert infix expression to postfix expression.
3. Calculate from postfix expression.

## Todo
- [ ] Code (As fast as I can...)
    - [ ] Handling invalid input from user.
        - [x] ~~Check is expression validate when click "=".~~
        - [x] ~~Check if input floating point is possible. ~~
        - [x] ~~Handling when input zero.~~
        - [ ] Output "NaN"when user try to divide by zero.
    - [ ] Handling buffer overflow.
    - [ ] Removing unnecessarily overlapped codes...
- [ ] Feature
    - [ ] Handling keyboard input
        - [ ] Block user input non number or not supported operator.
        - [ ] Support enter to calculate.
- [ ] UI 
    - [ ] Separating UI and logic by refactor to composite template.