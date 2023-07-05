def base62_encode(num):
    chars = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz"
    string = ""
    while num > 0:
        num, remainder = divmod(num, 62)
        string = chars[remainder] + string
    return string

print(base62_encode(2009215674938)) 