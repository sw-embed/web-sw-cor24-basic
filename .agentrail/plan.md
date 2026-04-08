1. Create correct pvm.s (full opcode dispatch extracted from pvmasm.s)
2. Rewrite runner.rs to use EmulatorCore + pvm.s (follow web-sw-cor24-pascal pattern)
3. Fix lib.rs compile errors
4. Verify cargo check passes
5. Test hello.bas in browser with playwright
6. Fix any runtime issues until output appears
7. Polish UI and deploy