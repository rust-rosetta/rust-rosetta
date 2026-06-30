#[cfg(test)]
#[rustfmt::skip]
mod tests {
    use assert_cmd::Command;
    use indoc::indoc;

    #[test]
    fn test_outputs() {
        let mut cmd = Command::cargo_bin("sierpinski-triangle").unwrap();
        cmd.arg("2").assert().success().stdout(indoc! {"
               *   
              * *  
             *   * 
            * * * *
        "});

        let mut cmd = Command::cargo_bin("sierpinski-triangle").unwrap();
        cmd.arg("3").assert().success().stdout(indoc! {"
                     *       
                    * *      
                   *   *     
                  * * * *    
                 *       *   
                * *     * *  
               *   *   *   * 
              * * * * * * * *
        "});
    }
}
