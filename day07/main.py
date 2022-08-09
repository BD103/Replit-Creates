import os
import hashlib
import time
import sys

from datetime import date


def generate_user_number() -> int:
    # Get username of person running repl, or return empty string
    username = os.environ.get("REPL_OWNER", "").encode()

    # If not logged in, generate random bytes instead
    if username == b"":
        username = os.urandom(32)

    # Append current date, so each day is a different fortune
    username += str(date.today()).encode()

    # Hash the username, so we get a constant size
    username_hash = hashlib.md5(username).digest()

    # Sum all of the bits in the hash
    username_value = 0

    for b in username_hash:
        username_value += b
    
    return username_value


def scroll(text: str, delay: float = 0.04, prefix: str | None = None, suffix: str = "\x1b[0m", end: str = "\n\n"):
    if prefix is not None:
        print(prefix, end="", flush=True)

    for c in text:
        print(c, end="", flush=True)
        time.sleep(delay)

    # Suffix + newline
    print(suffix, end=end, flush=True)


def begin_fortune_telling():
    username_value = generate_user_number()
    
    print("\nBefore we begin, let me ask you...")
    time.sleep(1)

    scroll("!!! A FEW QUESTIONS ABOUT YOURSELF !!!", prefix="\x1b[33m")

    _hair_color = input("What color is your hair?\n> ")
    _eye_color = input("What color are your eyes?\n> ")
    _nose_color = input("What color is your nose?\n> ")
    _toe_color = input("What color are your toes? (If multiple, please list with commas in between.)\n> ")

    for _ in range(3):
        print(end="\n")
        time.sleep(1)

    print("Thank you for your information.")
    time.sleep(2)

    print("It will now be...")
    time.sleep(1)

    scroll("!!! DISCARDED. !!!", delay=0.08, prefix="\x1b[31m")
    time.sleep(1)

    scroll("!!! YOUR FORTUNE IS ALREADY DECIDED. !!!", delay=0.08, prefix="\x1b[31m")
    time.sleep(1)

    scroll("!!! ANYTHING THAT YOU SAY HERE DOES NOT MATTER. !!!", delay=0.08, prefix="\x1b[31m")
    time.sleep(1)

    scroll("!!! FOR YOUR FATE HAS BEEN CHOSEN THE MOMENT YOU CLICKED RUN. !!!", delay=0.08, prefix="\x1b[31m")
    time.sleep(1)

    scroll("!!! YOUR FORTUNE IS... !!!", delay=0.2, prefix="\x1b[31m")
    time.sleep(2)

    is_50 = username_value % 2 == 0
    fortune_insert = (
        "Congratulations!" if is_50 else "                ",
        "   " if is_50 else "not",
    )
    
    scroll(f"!!! {fortune_insert[0]} YOU ARE {fortune_insert[1]} PART OF THE 50%. !!!", prefix="\x1b[34m")
    time.sleep(4)


def main():
    print("Hello, and welcome to my...")
    time.sleep(1)

    scroll("!!! FORTUNE TELLER THING OF EPICNESS-NESS WOO !!!", prefix="\x1b[35m")

    print("\nAre you ready for me to...")
    time.sleep(1)

    scroll("!!! READ YOUR FORTUNE OMG OMG OMG??? !!!", prefix="\x1b[32m")

    player_input = input("[type 'yes'] > ")

    if player_input == "yes":
        begin_fortune_telling()
    else:
        print("Hmmmmmm... that was not 'yes' exactly!\nCome back when you learn to follow instructions.", file=sys.stderr)
        sys.exit(2)
    
    print("Thank you for visiting my...")
    scroll("!!! FORTUNE TELLER THING OF EPICNESS-NESS WOO !!!", prefix="\x1b[35m")
    time.sleep(5)

    print("\n(Please come again!)")


if __name__ == "__main__":
    main()
