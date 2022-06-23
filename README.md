# TaskbarXI-Rust

An application to modify the Windows 11 Taskbar, written in Rust.

## TODO

Currently under development

## Contributing

- Ensure git hooks are configured:  
  `git config core.hooksPath .githooks`  
  For now git hooks will be utilized in favor of CI/CD due to reliance on Windows API's, window manager etc. that aren't available in a clean-room environment such as a Github Actions runner.

### Useful Tools

[Microsoft's Spy++](https://docs.microsoft.com/en-us/visualstudio/debugger/introducing-spy-increment?view=vs-2022) is a handy tool for inspecting window data (window handles, class names, caption names) and more  

## Attribution

**Thanks to the original contributors of [TaskbarXI](https://github.com/ChrisAnd1998/TaskbarXI): ❤️**  
[@ChrisAnd1998](https://github.com/ChrisAnd1998)  
[@PrincessAkira](https://github.com/PrincessAkira)  
[@dmitryaleshin](https://github.com/dmitryaleshin)  
[@officialLennox](https://github.com/officialLennox)
