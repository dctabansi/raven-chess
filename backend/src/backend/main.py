from backend import engine

def main():
    print("Hello from backend!")
    result = engine.add(10, 20)
    print(result)

if __name__ == "__main__":
    main()
