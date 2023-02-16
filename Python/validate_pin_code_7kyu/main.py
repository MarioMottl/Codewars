import re

def validate_pin(pin) -> bool:
    pin = str(pin)
    return True if re.match("^[0-9]{4}?$", pin) != None or re.match("^[0-9]{6}?$", pin) != None else False
    #return true or false


print(validate_pin(
    '098765
'))