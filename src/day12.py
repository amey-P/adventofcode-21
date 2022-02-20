
class Solver:
    def __init__(self, data):
        self.paths = {}
        self.data = data
        self.visited = set()

        self.prepare_paths()
        print(self.solve(part="1"))
        print(self.solve(part="2"))

    def prepare_paths(self):
        for d in self.data:
            l,r = d.split("-")
            if self.paths.get(l):
                self.paths[l].append(r)
            else:
                self.paths[l] = [r]
            if self.paths.get(r):
                self.paths[r].append(l)
            else:
                self.paths[r]=[l]

    def solve(self, curr_cave="start", part="1"):
        if (curr_cave=="end"):
            return 1
        if curr_cave.islower():
            self.visited.add(curr_cave)

        ways_count = sum([self.solve(cave, part) for cave in self.paths[curr_cave] if cave not in self.visited])
        ways_count += 0 if part!="2" else sum([self.solve(cave, cave) for cave in self.paths[curr_cave] if cave in self.visited and cave != "start"])

        if (curr_cave != part): self.visited.discard(curr_cave)
        return ways_count
