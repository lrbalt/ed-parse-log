### ed-parse-log
Parse the log files of the space flight simulator [Elite Dangerous](https://www.elitedangerous.com/). This repository contains some examples on how to use it.

### What can it do?
* It uses `serde_json` to parse a line from a log file. 
* It uses serde's `deny_unknown_fields` to be strict in parsing, i.e. it will fail on unexpected fields in a log line. 
* It does not integrate with any online database like [Inara](inara.cz), so all information comes from the parsed log lines alone. 
* It uses `Option` to handle new fields and other differences in the way log files are filled over time by different versions of Elite.

### Alternatives
After I worked on this crate I found [ed-journals](https://github.com/rster2002/ed-journals)

### Current status
* This crate can parse my own logs going back to june 2022
* it parses all test files from [ed-journals](https://github.com/rster2002/ed-journals)
* On my M2 Macbook it parses at ± 340 MB/s
* it needs further refactoring to improve and dry the data model

### How to run an example

There a several examples how to parse (in parallel) all logs in a specific directory and show information from the logs on de command line.

To just read all log files and show parse speeds

    cargo run --example read_all_logs --release -- /path/to/log/files  

If you want to see your progress

    cargo run --example progress --release -- /path/to/log/files  

You can list all community goals you are participating on or participated at

    cargo run --example community_goal --release -- /path/to/log/files  

You can see what you have learned about a system called "Shinrarta Dezhra" by using

    cargo run --example system_info --release -- /path/to/log/files "Shinrarta Dezhra"

And to see more information about a specific market, including status of colonisation and needed materials, you can find the id's using `system_info` above. For example "Jameson Memorial" is 128666762, so run

    cargo run --example market_info --release -- /path/to/log/files 128666762

### What if my log files do not parse
Each line is parsed seperately, so you can ignore the lines that fail. The examples keep track of all errors and will show the last five and stop, but you can choose to do things differenty of course.

You can help improve this crate by contributing the failing log lines in an Issue. But do make sure to anonimise the log line.