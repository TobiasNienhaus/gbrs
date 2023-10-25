import re

IN_OWN = "../dev/log/outoutout2.log"
IN_EXT = "../dev/log/working3-short.txt"

PAT_EXT = re.compile(
    r"^[\s\S]{6}(?P<address>[0-9a-fA-F]{4})\s(?P<exec>[a-zA-Z]+)[\s\S]+BC=(?P<BC>[0-9A-F]{4})\sDE=(?P<DE>[0-9A-F]{4})\sHL=(?P<HL>[0-9A-F]{4})\sAF=(?P<AF>[0-9A-F]{4})\sSP=(?P<SP>[0-9A-F]{4})\sPC=(?P<PC>[0-9A-F]{4})"
)
PAT_OWN = re.compile(
    r"^[\s\S]?(?P<address>[0-9a-fA-F]{4})[\s\S](?P<exec>[0-9a-fA-F]+)\sBC=(?P<BC>[0-9A-F]{4})\sDE=(?P<DE>[0-9A-F]{4})\sHL=(?P<HL>[0-9A-F]{4})\sAF=(?P<AF>[0-9A-F]{4})\sSP=(?P<SP>[0-9A-F]{4})\sPC=(?P<PC>[0-9A-F]{4})"
)


def main():
    own = {
        "address": [],
        "exec": [],
        "BC": [],
        "DE": [],
        "HL": [],
        "AF": [],
        "SP": [],
        "PC": [],
    }
    ext = {
        "address": [],
        "exec": [],
        "BC": [],
        "DE": [],
        "HL": [],
        "AF": [],
        "SP": [],
        "PC": [],
    }

    with open(IN_OWN, encoding="utf-16le") as fi:
        for line in fi:
            for match in PAT_OWN.finditer(line):
                for key in own.keys():
                    own[key].append(match.group(key))

    with open(IN_EXT) as fi:
        for line in fi:
            for match in PAT_EXT.finditer(line):
                for key in own.keys():
                    ext[key].append(match.group(key))

    cmp("address", ext, own)
    cmp("BC", ext, own)
    cmp("DE", ext, own)
    cmp("HL", ext, own)
    cmp("AF", ext, own)
    cmp("SP", ext, own)
    cmp("PC", ext, own)


def cmp(name, a, b):
    print(name)
    for i, (own, ext) in enumerate(zip(a[name], b[name])):
        if own != ext:
            print(f"{i}: {own} != {ext}")
            break


if __name__ == "__main__":
    main()
