import rusty_snek
import time

def idk(n: int) -> None:
    for i in range (n):
        a = 0.003*0.002 # i^6



def main() -> None:
    #print(rusty_snek.add(3,5))
    #print(rusty_snek.neuron_calculator([1.0, 2.0, 3.0, 4.0], [1.0, 2.0, 3.0, 4.0]))
    print()

    n = 1_000_000_000

    start = time.time()
    idk(n)
    end = time.time()
    print(f"Python function took {end - start:.6f} seconds")

    start = time.time()
    rusty_snek.idk(n)
    end = time.time()
    print(f"Rust function took {end - start:.6f} seconds")


if __name__ == '__main__':
    main()