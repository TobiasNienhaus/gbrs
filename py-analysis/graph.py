from matplotlib import pyplot as plt
import numpy as np
import re

PAT = re.compile(
    r"(?P<em>\d+)\s\((?P<em_perc>\d+\.\d+)\)\s\|\s(?P<draw>\d+)\s\((?P<draw_perc>\d+\.\d+)\)\s\|\s(?P<total>\d+)"
)
INPUT = "../dev/log/perf-unbound2.log"


def main():
    em = []
    em_perc = []
    draw = []
    draw_perc = []
    total = []

    with open(INPUT, encoding="utf-16le") as fi:
        for line in fi:
            for match in PAT.finditer(line):
                em.append(int(match.group("em")))
                em_perc.append(float(match.group("em_perc")))
                draw.append(int(match.group("draw")))
                draw_perc.append(float(match.group("draw_perc")))
                total.append(int(match.group("total")))

    def filter_deviance(ar, dev):
        avg = np.average(ar)
        _min = avg * (1 - dev)
        _max = avg * (1 + dev)
        return list(filter(lambda n: n >= _min and n <= _max, ar))

    fem = filter_deviance(em, 0.5)
    fem_perc = filter_deviance(em_perc, 0.5)
    fdraw = filter_deviance(draw, 0.5)
    fdraw_perc = filter_deviance(draw_perc, 0.5)
    ftotal = filter_deviance(total, 0.5)

    # plt.plot(np.arange(len(fem_perc)), fem_perc)

    ftotal_ms = [n / 1000000 for n in ftotal]
    ftotal_s = [n / 1000000000 for n in ftotal]
    ftotal_fps = [1000 / n for n in ftotal_ms]
    # plt.plot(ftotal_fps)
    plt.stackplot(np.arange(len(em_perc)), em_perc, [1 - n for n in em_perc])

    plt.show()


if __name__ == "__main__":
    main()
