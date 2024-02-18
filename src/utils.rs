pub fn clear_screen() {
    /*! x1B = Escape
     * [2J = clear screen
     * x1B[1;1H = Move cursor x/y */
    print!("\x1B[2J\x1B[1;1H");
}
