    extern crate colored; 

    use colored::Colorize;

    fn main() {
        println! ("{} {} {} {}", "                 R RR RR                 ".bright_red(), " 02sh".bright_red(), "~".white(), "git version 2.30.2".bright_red());
        println! ("{} {}", "              R RRRRRRRR R           R   ".bright_red(), " __________________________".white());
        println! ("{} {} {}", " R RR       R RRRRRRRRRRRRR R       RR   ".bright_red(), " Project:".bright_red(), "rust (10 branches, 111 tags)".white());
        println! ("{} {} {}", "rR RRR    R RRRRRRRRRRRRRRRRRR R   RRR R ".bright_red(), " HEAD:".bright_red(), "9044245 (master, orgin/master)".white());
        println! ("{} {} {}", "RRR RR   RRRRRRRRRRRRRRRRRRRRRRRR  RRRRR ".bright_red(), " Pending:".bright_red(), "3+".white());
        println! ("{} {} {}", " RRRRR  RRRRRRRRRRRRRRRRRRRRRRRRR  RRRR  ".bright_red(), " Version:".bright_red(), "1.64.0".white());
        println! ("{} {} {}", "  RRR RRRRRRRRRRRRRRRRRRRRRRRRRRRRR RR   ".bright_red(), " Created:".bright_red(), "12 years ago".white());
        println! ( "{} {} {} {} {} {} {}", "    R  RRRRRRRRRR".bright_red(), "=".white(), "RR".bright_red(), "=".white(), "RRRRRRRRRRRR".bright_red(), "     Languages:".bright_red(), "Rust (97.4 %) Python (0.5 %)".white());
        println! ( "{} {} {} {} {} {}", "     RRRRRRRRRRRR".bright_red(),"=".white(),"RR".bright_red(),"=".white(),"RRRRRRRRRRR".bright_red(), "                 JavaScript (0.4 %) CSS (0.3 %)".white());
        println! ("{} {}", "      RRRRRRRRRRR   RR   RRRRRRRRRRR     ".bright_red(), "            C++ (0.3 %) Markdown (0.3 %)".white());
        println! ("{} {}", "     RR==RRRRRRRRRRRRRRRRRRRRRR===RR     ".bright_red(), "            Other (0.7 %)".white());
        println! ("{} {} {}", "     RR =  ==RRRRRRR  RRRRRR==  = RR     ".bright_red(), " Authors:".bright_red(), "5% Brian Anderson 5259".white());
        println! ("{} {}", "      RR =     ===========     = RR      ".bright_red(), "          4% Niko Natsakis 4074".white());
        println! ("{} {}", "       RR                       R        ".bright_red(), "          3% Alex Crichton 3616".white());
        println! ("{} {} {}", "        R                      R         ".bright_red(), " Last Change:".bright_red(), "a day ago".white());
        println! ("{} {} {}", "         R                               ".bright_red(), " Contributors:".bright_red(), "4068".white());
        println! ("{} {} {}", "                                         ".bright_red(), " Repo:".bright_red(), "https://github.com/rust-lang/rust".white());
        println! ("{} {} {}", "                                         ".bright_red(), " Commits:".bright_red(), "108408".white());
        println! ("{} {} {}", "                                         ".bright_red(), " Lines of code:".bright_red(), "1001429".white());
        println! ("{} {} {}", "                                         ".bright_red(), " Size:".bright_red(), "63.53 MiB (29704 files)".white());
        println! ("{} {} {}", "                                         ".bright_red(), " License:".bright_red(), "Apache-2.0, MIT".white());

    }
