
def main():
    file = open("../scenario.txt", "r")

    header = file.readline().split(",")
    header = map(int, header)
    (number_of_objects, max_weight, max_size) = tuple(header)

    items = []

    for item in file.readlines():
        item = item.split(",")
        item = map(int, item)

        (weight, size, cost) = tuple(item)

        value = cost / (weight + size)

        items.append((value, weight, size, cost))

    assert number_of_objects == len(items)

    items.sort(key=lambda x: x[0], reverse=True)

    total_weight = 0
    total_size = 0
    total_cost = 0

    for value, weight, size, cost in items:
        total_weight += weight
        total_size += size

        if total_weight > max_weight or total_size > max_size:
            break

        total_cost += cost

    print(total_cost)


if __name__ == "__main__":
    main()
