import re

with open("data/4.1.csv","r") as f:
    data = f.readlines()

content = [x.strip() for x in data] 
content.append("")
res = []
port_list = []
ports = []
for l in content:
    if str(l) == "":
        res.append(port_list)
        port_list = []
    else:
        port_list.append(l)
for l in res:
    port = ""
    for subl in l:
        port +=  " " + subl
    ports.append(port)

dicts = []
for port in ports:
    s = port.split(" ")
    s = [p.split(":") for p in s]
    s = list(filter(lambda x: not x[0] == '',s))
    # print(s)
    dicts.append({
       p[0]:p[1] for p in s
    })

def valid_fields(port):
    fields = ["byr", "iyr", "eyr", "hgt","hcl", "ecl","pid"]
    return all(f in port.keys() for f in fields)

def valid_entries(port):
    hgt_res = False
    if port['hgt'].endswith("cm"):
        hgt_res = 150 <= int(port['hgt'][:-2]) <= 193
    elif port['hgt'].endswith("in"):
        hgt_res = 59 <= int(port['hgt'][:-2]) <= 76

    byr_res = 1920 <= int(port['byr']) <= 2002
    iyr_res = 2010 <= int(port['iyr']) <= 2020 
    eyr_res = 2020 <= int(port['eyr']) <= 2030 
    pid_res = re.match("[0-9]{9}", port['pid']) is not None

    hcl = port['hcl'].split("#")
    print(port['hcl'])
    print(hcl)
    if len(hcl) < 2:
        hcl_res = False
    elif len(hcl[1]) == 6:
        hcl_res = re.match("[0-9a-f]{6}",hcl[1]) is not None
    else:
        hcl_res = False

    ecl_res = port['ecl'] in ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]


    return all([hgt_res,byr_res,iyr_res, eyr_res, hcl_res, ecl_res, pid_res])

res = filter(valid_fields,dicts)
res = filter(valid_entries,res)

print(len(list(res)))