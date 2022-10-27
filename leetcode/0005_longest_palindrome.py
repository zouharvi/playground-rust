def longestPalindrome(s: str) -> str:
    # all individual characters are palindromes, also add phantom positions
    palindromes = [
        [c_i, c_i] for c_i, c in enumerate(s)
    ] + [
        [c_i, c_i - 1] for c_i, c in enumerate(s)
    ]
    next_palindromes = []
    for possible_length in range(len(s)//2):
        for p_i, p_j in palindromes:
            # attempt to extend both sides
            if p_i > 0 and p_j < len(s) - 1:
                if s[p_i - 1] == s[p_j + 1]:
                    next_palindromes.append((p_i-1, p_j+1))

        if len(next_palindromes) == 0:
            break
        else:
            palindromes = next_palindromes
            next_palindromes = []

    max_palindrome = max(palindromes, key=lambda x: x[1]-x[0])
    return s[max_palindrome[0]:max_palindrome[1]+1]


print(longestPalindrome("bdcacdax"))
print(longestPalindrome("bb"))
print(longestPalindrome("uwqrvqslistiezghcxaocjbhtktayupazvowjrgexqobeymperyxtfkchujjkeefmdngfabycqzlslocjqipkszmihaarekosdkwvsirzxpauzqgnftcuflzyqwftwdeizwjhloqwkhevfovqwyvwcrosexhflkcudycvuelvvqlbzxoajisqgwgzhioomucfmkmyaqufqggimzpvggdohgxheielsqucemxrkmmagozxhvxlwvtbbcegkvvdrgkqszgajebbobxnossfrafglxvryhvyfcibfkgpbsorqprfujfgbmbctsenvbzcvypcjubsnjrjvyznbswqawodghmigdwgijfytxbgpxreyevuprpztmjejkaqyhppchuuytkdsteroptkouuvmkvejfunmawyuezxvxlrjulzdikvhgxajohpzrshrnngesarimyopgqydcmsaciegqlpqnclpwcjqmhtmtwwtbkmtnntdllqbyyhfxsjyhugnjbebtxeljytoxvqvrxygmtogndrhlcmbmgiueliyfkkcuypvvzkomjrfhuhhnfbxeuvssvvllgukdolffukzwqaimxkngnjnmsbvwkajyxqntsqjkjqvwxnlxwjfiaofejtjcveqstqhdzgqistxwsgrovvwgorjaoosremqbzllgbgrwtqdggxnyvkivlcvnv"))
print(longestPalindrome("zeusnilemacaronimaisanitratetartinasiaminoracamelinsuez"))