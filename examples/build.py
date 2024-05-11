import re
import json
import base64
import subprocess

target = "notes.html"
settings_file = "settings.json"
out = "out.js"


includes = re.compile(r'<include>(.*)</include>')
settings = re.compile(r'<setting>(.*)</setting>')
base64s = re.compile(r'<base64>(.*)</base64>')
systems = re.compile(r'<system>(.*)</system')

def minify(text:str)->str:
    return text.replace("\n","").replace("\'","\"")

def render(target:str,files:list[str]=[]) ->str :
    with open(target,"r") as f_target:
        target_text = f_target.read()
        for include in re.findall(includes,target_text):
            if include in files:
                exit(f"while rendering {target} found recursive include {include}")
            else:
                include_text = minify(render(include,files + [target]))    
                target_text = target_text.replace(f"<include>{include}</include>",include_text)
        for setting in re.findall(settings,target_text):
            f,n = setting.split(":")
            s = json.load(open(f+".json","r"))
            target_text = target_text.replace(f"<setting>{setting}</setting>",s[n])
        for b64 in re.findall(base64s,target_text):
            with open(b64, "rb") as i_file:
                encoded_string = base64.b64encode(i_file.read()).decode('utf-8')
                target_text = target_text.replace(f"<base64>{b64}</base64>",encoded_string)
        for system in re.findall(systems,target_text):
            result = subprocess.run(system.split(" "), stdout=subprocess.PIPE).stdout.decode('utf-8')
            target_text = target_text.replace(f"<system>{system}</system>",result)
        
        return target_text


if __name__ == "__main__":    
    print(render(target))
