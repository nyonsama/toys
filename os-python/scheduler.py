import random
from typing import List
from copy import deepcopy

#####################
#  MyProcess Class  #
#####################
"""
   Attribute:
    :param pid: process's identifier
    :param priority: process's priority
    :param used_time: the time that this process has spent
    :param max_time: the time that this process need 
    :param status: process's status
   Method:
    :function run: run and return the status
"""


class MyProcess:
    def __init__(self, pid: int, priority: int, time: int) -> None:
        self.pid: int = pid
        self.priority: int = priority
        self.used_time: int = 0
        self.max_time: int = time
        self.status: str = "ready"

    def __str__(self) -> str:
        return 'pid:{} priority:{} used_time:{} total_time:{} status:{}'.format(self.pid, self.priority, self.used_time, self.max_time, self.status)

    def run(self) -> str:
        '运行进程,返回这个进程运行后的状态'
        print('running process(pid:{})'.format(self.pid))
        if(self.priority != 0):
            self.priority -= 1
        self.used_time += 1
        if self.used_time == self.max_time:
            self.status = "stopped"
            print('  process(pid:{}) stopped'.format(self.pid))
        else:
            print('  process(pid:{}) halted'.format(self.pid))
        return self.status


###############
#  Scheduler  #
###############
"""
   Method:
    __init__: initialize a process queue
    add: add the process to the queue
    print_queue: print the current queue
"""


class Scheduler:
    def __init__(self) -> None:
        self.queue: List[MyProcess] = []
        self.count: int = 0
        self.countlist: List[int] = []
        self.responselist: List[int] = []

    def add(self, p: MyProcess) -> None:
        self.queue.append(p)

    def print_queue(self) -> None:
        print('current queue:')
        if self.queue:
            for p in self.queue:
                print('  {}'.format(p))
        else:
            print('  empty')

    def print_countlist(self) -> None:
        print('turn over time list:')
        print(self.countlist)


##################
#  HpfScheduler  #
##################

"""Describe the Highest Priority First Schedule"""


class HpfScheduler(Scheduler):
    def run(self) -> None:
        while self.queue:
            # 取得队列中拥有最高优先级的进程
            p: MyProcess = max(self.queue, key=lambda x: x.priority)
            if p.used_time == 0:
                self.responselist.append(self.count)
            self.count += 1
            if p.run() == "stopped":
                self.queue.remove(p)
                self.countlist.append(self.count)
                self.print_countlist()
            self.print_queue()


#################
#  RrScheduler  #
#################

"""Describe the Round Robin Schedule"""


class RrScheduler(Scheduler):
    def run(self) -> None:
        while self.queue:
            p: MyProcess = self.queue.pop(0)
            if p.used_time == 0:
                self.responselist.append(self.count)
            self.count += 1
            if p.run() == "ready":
                self.queue.append(p)
            else:
                self.countlist.append(self.count)
                self.print_countlist()
            self.print_queue()


##########
#  Test  #
##########

class Test:
    def __init__(self) -> None:
        self.processes: List[MyProcess] = []
        self.turn_over_time: float = 0  # 平均周转时间
        self.system_throughput: float = 0  # 系统平均吞吐量
        self.response_time: float = 0  # 平均响应时间

    def generate(self):
        for i in range(5):
            self.processes.append(
                MyProcess(i+1, random.randint(1, 20), random.randint(5, 20)))

    def caculate_avg_tat(self, countlist: List[int]) -> float:
        self.turn_over_time = sum(countlist)/len(countlist)
        print('average_turn_around_time:', self.turn_over_time)
        return self.turn_over_time

    def caculate_tp(self, countlist: List[int]) -> float:
        self.system_throughput = len(countlist)/countlist.pop()
        print('system_throughput:', self.system_throughput)
        return self.system_throughput

    def caculate_avg_tp(self, countlist: List[float]) -> float:
        self.system_throughput = sum(countlist)/len(countlist)
        print('average_system_throughput:', self.system_throughput)
        return self.system_throughput

    def caculate_avg_rt(self, responselist: List[int]) -> float:
        self.response_time = sum(responselist)/len(responselist)
        print('average_response_time', self.response_time)
        return self.response_time


if __name__ == '__main__':
    avg_list1: List[float] = []
    avg_list2: List[float] = []
    throughput_list1: List[float] = []
    throughput_list2: List[float] = []
    res_list1: List[float] = []
    res_list2: List[float] = []
    for i in range(100):
        test = Test()
        test.generate()

        print('start HPF schedule'.center(60, '-'))
        hpf = HpfScheduler()
        for p in test.processes:
            hpf.add(deepcopy(p))
        hpf.run()

        print('start RR schedule'.center(60, '-'))
        rr = RrScheduler()
        for p in test.processes:
            rr.add(deepcopy(p))
        rr.run()

        avg_list1.append(test.caculate_avg_tat(hpf.countlist))
        avg_list2.append(test.caculate_avg_tat(rr.countlist))
        throughput_list1.append(test.caculate_tp(hpf.countlist))
        throughput_list2.append(test.caculate_tp(rr.countlist))
        res_list1.append(test.caculate_avg_rt(hpf.responselist))
        res_list2.append(test.caculate_avg_rt(rr.responselist))

    print('Test result:'.center(60, '-'))
    print('hpf scheduler')
    test.caculate_avg_tat(avg_list1)
    test.caculate_avg_tp(throughput_list1)
    test.caculate_avg_rt(res_list1)
    print()
    print('rr scheduler')
    test.caculate_avg_tat(avg_list2)
    test.caculate_avg_tp(throughput_list2)
    test.caculate_avg_rt(res_list2)
    print(''.center(60, '-'))
