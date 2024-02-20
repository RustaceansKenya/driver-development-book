# Monitoring and Logging

Monitoring is the act of fetching runtime data from the chip... and presenting it in a useful way. The data could be the CPU register values, stack values, other RAM-stored values... anything that provides info that may give insight on how the program works.  

You can monitor using custom-made tools or third-party tools. Even gdb can help you inspect runtime values, so it counts as some kind of monitoring tool.  


Logging is slightly different from monitoring, but they do the same thing in a different way. Logging fetches data from the runtime too... but it fetches predefined data or descriptions in an organized way.  

Logging is closer to the 'println!' kind of debugging. But in this case, instead of writing random 'print me' lines, you classify those print statements in a relevant way... in a classified way.  

For example, you may tag some print statements as `errors`, `warnings`, `just_info`, `debugging_info`.  
From there, you can use your logging tool to regulate how the 'log statements' get compiled, filtered and even how they get displayed.  

There are some logging tools that allow `module level` control of which logging info gets processed and displayed.  