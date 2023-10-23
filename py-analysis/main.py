import re

IN_OWN = "../dev/log/out4.log"
IN_EXT = "../dev/log/working2-safety-short.txt"

PAT_EXT = re.compile(r"^[\s\S]{6}(?P<address>[0-9a-fA-F]{4})\s(?P<exec>[a-zA-Z]+)")
PAT_OWN = re.compile(r"^[\s\S]?(?P<address>[0-9a-fA-F]{4})[\s\S](?P<exec>[0-9a-fA-F]+)")


def main():
    addresses_own = []
    exec_own = []

    with open(IN_OWN, encoding="utf-16le") as fi:
        for line in fi:
            for match in PAT_OWN.finditer(line):
                addresses_own.append(match.group("address"))
                exec_own.append(match.group("exec"))

    addresses_ext = []
    exec_ext = []

    with open(IN_EXT) as fi:
        for line in fi:
            for match in PAT_EXT.finditer(line):
                addresses_ext.append(match.group("address"))
                exec_ext.append(match.group("exec"))

    for i, (own, ext) in enumerate(zip(addresses_own, addresses_ext)):
        print(f"{own} | {ext}")
        if own != ext:
            print(f"{i}: {own} != {ext}")
            break


if __name__ == "__main__":
    main()
