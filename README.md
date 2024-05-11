# castle 

```
<body>
    an include tag will dump the content of a file here :
    <include>README.md</include>
    a setting tag will pull a varible from a json 
    < setting > filename:var name
    <setting>plugin.json:buttonCode</setting>
    a base64 tage dumps a base64 of the file 
    <base64>build.py</base64>
    system tag dumps the output of a system command 
    <system>python --version</system>
    <system>uname -a</system>
    this next line shouuld error 
    <system>ecsho <system>this shouls not run</system></system>
    a macro tag with run a lua script file with arguments
    <macro>test.lua: these are args 234234</macro>
    a macro tag with run a lua script file
    <macro>test.lua</macro>
    a lua tag runs lua code 
    <lua>1+2</lua>
    <setting>plugin.json:this is</setting>
    a blueprint tag import a tera template simular to Jinja 
    <blueprint>working.html:name=john car=red </blueprint>
    https://github.com/Keats/tera/tree/master?tab=readme-ov-file
</body>

```