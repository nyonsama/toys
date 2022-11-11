from typing import List
from copy import deepcopy
Resource = List[int]

#####################
#  MyProcess Class  #
#####################
"""
   Attribute:
    :param pid: process's identifier
    :param need: the resource that process need until now
    :param total: the resource that process totally need
    :param own: the resource that process has have
    :param requests: the resource list that process need by time
   Method:
    :function get_request: get the resource that need currently
    :function allocate: allocate the resource (own+,need-)
    :function is_satisfied: check the need whether == 0
"""


class MyProcess:
    def __init__(self, pid: int, total: Resource, own: Resource, need: Resource, requests: List[Resource]) -> None:
        self.pid = pid
        self.need = need
        self.total = total
        self.own = own
        self.status: str = "ready"
        self.requests = requests

    def __str__(self) -> str:
        return 'pid:{} need:{} total:{} own:{} status:{}'.format(self.pid, self.need, self.total, self.own, self.status)

    def get_request(self) -> Resource:
        return self.requests[0]

    def allocate(self, res: Resource) -> None:
        '给这个进程分配资源'
        self.requests.pop(0)
        for i in range(len(self.total)):
            self.own[i] += res[i]
            self.need[i] -= res[i]

    def is_satisfied(self) -> bool:
        for i in self.need:
            if i != 0:
                return False
        return True

#################
#  MyAllocater  #
#################


class MyAllocator():
    def __init__(self, res: Resource, procs: List[MyProcess]) -> None:
        self.res = res
        self.procs = procs

    def allocate_for(self, proc: MyProcess) -> Resource:
        "返回分配了多少资源；分配失败则返回None"
        pass

    def free(self, proc: MyProcess) -> Resource:
        proc.status = 'stopped'
        for i in range(3):
            self.res[i] += proc.own[i]
        return proc.own

    def run(self) -> None:
        timer = 0
        while self.procs:
            timer += 1
            p = self.procs[0]
            if p.is_satisfied():
                print("pid:{} is satisfied.free it.".format(p.pid))
                self.free(p)
                self.procs.pop(0)
                continue

            res = self.allocate_for(p)
            if res:
                print('allocate resource A:{}, B:{}, C:{} for pid:{}'.format(
                    res[0], res[1], res[2], p.pid))
            else:
                print(
                    "allocate resource for pid:{} failed. resources are not enough.".format(p.pid))
            self.procs.append(self.procs.pop(0))

            if timer > 60:
                print('deadlock occured!')
                break

        if not self.procs:
            print('all processes are satisfied.')


###################
#  BankAllocater  #
###################

class BankerAllocator(MyAllocator):
    def allocate_for(self, proc: MyProcess) -> Resource:
        assert self.is_safe()
        request = proc.get_request()

        # 分配资源
        for i in range(len(proc.total)):
            proc.own[i] += request[i]
            proc.need[i] -= request[i]

        backup = deepcopy(self.res)
        for i in range(len(request)):
            self.res[i] -= request[i]
        if not self.is_safe():
            # 取消分配
            for i in range(len(proc.total)):
                proc.own[i] -= request[i]
                proc.need[i] += request[i]

            self.res = backup
            proc.status = 'waiting'
            return None

        proc.requests.pop(0)
        proc.status = 'ready'
        return request

    def is_safe(self) -> bool:
        for p in self.procs:
            for i in range(len(p.need)):
                if p.need[i] > self.res[i]:
                    break
            else:
                return True
        return False


#####################
#  RandomAllocater  #
#####################

class RandomAllocator(MyAllocator):
    def allocate_for(self, proc: MyProcess) -> Resource:
        request = proc.get_request()
        for i in range(3):
            if self.res[i] < request[i]:
                proc.status = 'waiting'
                return None

        for i in range(3):
            self.res[i] -= request[i]
        proc.allocate(request)
        proc.status = 'ready'
        return request


if __name__ == '__main__':
    processes: List[MyProcess] = [
        MyProcess(0, [7, 5, 3], [0, 1, 0], [7, 4, 3],
                  requests=[[2, 1, 3], [5, 3, 0]]),
        MyProcess(1, [3, 2, 2], [2, 0, 0], [1, 2, 2],
                  requests=[[1, 2, 2]]),
        MyProcess(2, [9, 0, 2], [3, 0, 2], [6, 0, 0],
                  requests=[[2, 0, 0], [4, 0, 0]]),
        MyProcess(3, [2, 2, 2], [2, 1, 1], [0, 1, 1],
                  requests=[[0, 1, 1]]),
        MyProcess(4, [4, 3, 3], [0, 0, 2], [4, 3, 1],
                  requests=[[1, 0, 0], [3, 3, 1]]),
    ]

    print('start banker allocator'.center(60, '-'))
    BankerAllocator(res=[3, 3, 2], procs=deepcopy(processes)).run()
    print('\n'+'start random allocator'.center(60, '-'))
    RandomAllocator(res=[3, 3, 2], procs=deepcopy(processes)).run()
