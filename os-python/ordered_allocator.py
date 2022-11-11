from typing import List
from copy import copy, deepcopy


#####################
#  MyProcess Class  #
#####################
"""
   Attribute:
    :param pid: process's identifier
    :param cur: the current resource number
    :param last: the last resource number 
    :param status: process's status
   Method:
    :function get_requests: get the resource list that need 
    :function allocate: allocate the resource (own+,need-)
    :function is_satisfied: check the need whether == 0
    :function update: update the `curl` and `last`
"""
class MyProcess():
    def __init__(self, pid: int, requests: List[int]) -> None:
        self.pid = pid
        self.cur: int = requests[0]
        self.last: int = -1
        self.requests = requests
        self.status: str = 'ready'

    def __str__(self) -> str:
        return 'pid:{} current waiting resource:{} last request:{} status:{}'.format(self.pid, self.cur, self.last, self.status)

    def allocate(self, res_id: int) -> None:
        # self.last = self.cur
        # self.cur = res_id
        self.requests.remove(res_id)

    def get_requests(self) -> List[int]:
        return self.requests

    def is_satisfied(self) -> bool:
        if self.requests:
            return False
        return True

    def update(self) -> None:
        '更新cur与last'
        self.last = self.cur
        if self.requests:
            self.cur = self.requests[0]
        else:
            self.cur = -1


#################
#  MyAllocater  #
#################

class MyAllocator():
    def __init__(self) -> None:
        self.record: List[int] = [-1]*10  # 索引是资源号，内容是拥有这个资源的pid

    def allocate_for(self, p: MyProcess) -> List[int]:
        pass

    def free(self, p: MyProcess) -> List[int]:
        p.status = 'stopped'
        freed: int = []
        for i in range(len(self.record)):
            if self.record[i] == p.pid:
                self.record[i] = -1
                freed.append(i)
        return freed

    def run(self, procs: List[MyProcess]) -> None:
        timer = 0
        while procs:
            timer += 1
            p = procs.pop(0)
            if p.is_satisfied():
                print("pid:{} is satisfied.free it.".format(p.pid))
                self.free(p)
                continue

            res = self.allocate_for(p)
            if res:
                print('allocate resource {} for pid:{}'.format(res, p.pid))
            else:
                print("allocate resource [{}] for pid:{} failed.resource is not enough.".format(
                    p.cur, p.pid))
            procs.append(p)

            if timer > 40:
                print('deadlock occured!')
                break

        if not procs:
            print('all processes are satisfied.')


######################
#  OrderedAllocater  #
######################

class OrderedAllocator(MyAllocator):
    def __init__(self) -> None:
        super().__init__()

    def allocate_for(self, p: MyProcess) -> List[int]:
        '返回分配了哪些资源序号，分配失败会返回None'
        requests = sorted(copy(p.get_requests()), reverse=True)

        for res_id in requests:
            # 检查所有序号比请求的资源大的资源是否被占用
            if res_id < p.cur:
                break
            owner_id = self.record[res_id]
            if owner_id != p.pid and owner_id != -1:
                # 已经被占用了
                p.status = 'waiting'
                return None

        allocated: List[int] = []
        for res_id in requests:
            if res_id < p.cur:
                break
            self.record[res_id] = p.pid
            p.allocate(res_id)
            allocated.append(res_id)
        p.status = 'ready'
        p.update()
        return allocated

    def free(self, p: MyProcess) -> List[int]:
        return super().free(p)

    def run(self, procs: List[MyProcess]) -> None:
        return super().run(procs)

####################
#  BlindAllocater  #
####################

class BlindAllocator(MyAllocator):
    def __init__(self) -> None:
        super().__init__()

    def allocate_for(self, p: MyProcess) -> List[int]:
        '返回分配了哪些资源序号，分配失败会返回None'
        requests = p.get_requests()

        owner = self.record[p.cur]
        if owner != p.pid and owner != -1:
            # 这个资源已经被占用了
            p.status = 'waiting'
            return None

        self.record[p.cur] = p.pid
        p.allocate(p.cur)
        p.status = 'ready'
        p.update()
        return [p.last]

    def free(self, p: MyProcess) -> List[int]:
        return super().free(p)

    def run(self, procs: List[MyProcess]) -> None:
        return super().run(procs)


if __name__ == '__main__':
    processes = [
        MyProcess(0, [1, 5, 3, 0, 6, 8, 7]),
        MyProcess(1, [9, 2, 5, 4, 3, 1]),
        MyProcess(2, [1, 4, 6, 9, 3, 5, 8, 2, 0])
    ]
    print('start ordered allocator'.center(60, '-'))
    OrderedAllocator().run(deepcopy(processes))
    print('\n'+'start blind allocator'.center(60, '-'))
    BlindAllocator().run(deepcopy(processes))
