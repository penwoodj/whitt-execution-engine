# Task: System Report Script

Write a bash script named sysreport.sh that gathers system information and writes a markdown report to a file whose path is given as the first argument (default ./sysreport.md). The report must contain these markdown sections: "## OS" (OS name and version), "## Uptime", "## Disk" (df -h summary of the root filesystem), "## Memory" (total and available), and "## Top Processes" (top 5 processes by CPU). The script must work on macOS and Linux, must not require root, and must print "REPORT WRITTEN: <path>" on success. Include usage comments at the top of the script.
