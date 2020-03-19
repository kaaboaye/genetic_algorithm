from glob import glob
import matplotlib.pyplot as plot
import numpy as np


def read_file(filename):
    results = open(filename) \
        .read() \
        .split("\n")

    results = map(lambda x: int(x), results)
    results = list(results)
    return np.asarray(results, dtype=int)


def average_results(results_list):
    generations = results_list[0].size
    universes_number = len(results_list)

    acc = np.zeros(generations, dtype=int)

    for results in results_list:
        acc = acc + results

    return acc / universes_number


def main():
    files = glob("res_**_**")

    files_dict = {}

    for file in files:
        [_, param, _] = file.split("_")

        iterations = files_dict.get(param, [])
        iterations.append(file)
        files_dict[param] = iterations

    for param, iterations in files_dict.items():
        results_list = map(read_file, iterations)
        results_list = list(results_list)

        results = average_results(results_list)

        plot.plot(results, label=param)

    plot.xlabel("Generation")
    plot.ylabel("Best individual")
    plot.legend(loc='lower right')
    plot.tight_layout()
    plot.ylim(ymin=0)
    plot.xlim(xmin=0)
    plot.show()


if __name__ == "__main__":
    main()
