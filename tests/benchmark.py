import listie
import random

rlist = [random.randint(1, 99999) for iter in range(1000)]

def sort_rs(l):
    return listie.sort(l)

def sort_py(l):
    return l.sort()

def test_sort_rs(benchmark):
    res = benchmark(sort_rs, rlist)

def test_sort_py(benchmark):
    res = benchmark(sort_py, rlist)

def dedup_rs(l):
    return listie.dedup(l)

def dedup_py(l):
    return set(l)

def test_dedup_rs(benchmark):
    res = benchmark(dedup_rs, rlist)

def test_dedup_py(benchmark):
    res = benchmark(dedup_py, rlist)
