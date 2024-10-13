       /*
        //euclid's formula:
            //a + b + c = 1000
            //a = m.pow(2) - n.pow(2), b = 2mn, c = m.pow(2) + n.pow(2)
            //m.pow(2) - n.pow(2) + 2mn + m.pow(2) + n.pow(2) = 1000
            //m.pow(2) + 2mn + m.pow(2) = 1000?
            //2m.pow(2) + 2mn = 1000?
            // need to learn reduction of common factors I guess!
            // in this case the common factor between 2m.pow(2) and 2mn is 2m
            // divide both sides by the common factor 2m
            // 2m.pow(2) becomes 2m * m since 2m.pow(2) / 2mn = m ???
            // 6 + 12 = (6 x 1 + 6 x 2)
            //thinking again,   
            //                  10 + 20 + 30 = 10 x 1 + 10 x 2 + 10 x 3
            //...               60 = 10 (1 + 2 + 3)
            //...               60 = 60! huh!
            //...               --- factoring out products I need practice
            //...               1000 = 2m.pow(2) + 2mn
            //...               500 = m^2 + mn?
            //...               1000 =  2m.pow(2) + 2mn
            //...               1000 = 2 * m * m + 2 * m * n
            //...               1000 = 2m(m + n) !
            //...               500 = m(m + n)
            //...               m, and n being factors of 500
            //...               500, 250, 125, 100, 50, 25, 20, 10, 5, 4, 2
            //...               so plugin for formula?
            //...               //m = 20, n = 10
            //...               500 != 20(20 + 10) 500 != 20(30)
            //...               //m = 20, n = 5
            //...               500 = 20(20 + 5) = 25 * 20 = 500
            //...               a = 20^2 - 5^2, b = 2 * 20 * 5, c = m^2 + n^2
            //...               a = 375, b = 200, c = 425, a + b + c = 1000 = 375 + 425 + 200
            //...               yah, so a * b * c = 375 * 425 * 200
            //...               abc = 31875000
            //...
            //... there is an absurdly faster way to solve this
            //... steps in english:
            //...
            //...   solving for a + b + c = 1000, where a, b, and c are derived from euclid's formula
            //...   a + b + c = 1000 is equivalent to 2m(m + n) = 1000
            //...   after finding the divisors of 1000, search through them and solve for a, b, and c where the sum of a, b, c is 1000
            //...   in the passing case find the product of abc for the answer
            //...   the fast way would then be...
            notes
                x pythagorean triples are made up of three positive integers (whole numbers), such that (a * a + b * b = c * c)
                x for triple (a, b, c), if (a^2 + b^2 = c^2) is true, is a pythagorean triple
                x (3, 4, 5), which would be (3 * 3 + 4 * 4 = 5 * 5), or (9 + 16 = 25), or (25 = 25)
                x additionally, for integer k, (ak, bk, ck) = the next triple
                x a primitive triple is one whose components are coprime, (3,4,5) are coprime
                    - coprime integers are integers that only share a common divisor of 1
                        10 has divisors of 10, 5, 2, 1
                        13 has only 13, and 1
                        since the gcd(10, 13) is 1, then 10 and 13 are coprime
                x every right triangle has a side lengths satisfying the pythagorean theorum, but not all triangles
                x non-integer sides do not form triples
                x a = b = 1, and c = sqrt(2), since sqrt(2) is not an integer or ratio of integers, then this isn't a triple
                    - more to that point, since sqrt(2) is an irrational number, 1 and sqrt(2) have no common multiples
                x there are 16 primitive pythagorean triples up to 100
                x (3, 4, 5) is a prim triple, but (3k, 4k, 5k) is not in any case
                x euclid's forumla; 
                    - a = m^2 - n^2, b = 2mn, c = m^2 + n^2
                    - given: m = 2, and n = 1
                        a = 2.pow(2) - 1.pow(2) = 3
                        b = 2 * 2 * 1 = 4
                        c = 2.pow(2) + 1.pow(2) = 5
                            (3, 4, 5)
                    - the triple generated is only primited if m and n are coprime, and only one of them is even
                    - if both m and n are odd, all a, b, and c will be even, and the triple won't be primitive (divisble by at 1, and 2)
                        however, dividing the generated result will yield a primitive triple when divided by 2
                        so: m = 7, n = 5
                        a = 7^2 - 5^2, b = 2 * 7 * 5, c = 7^2 + 5^2
                        triple by 2 (24, 70, 74) / 2
                        suspected primitive triple: 12, 35, 37
                            prime factors..? 12: 1, 2, 3, 4, 6 35: 1, 5, 7 37: 1 - correct!
                            (12, 35, 37) is a primitive pythagorean triple
                x euclid's formula doesn't produce all possible non-primitive triples unless modified:
                    each input must be multiplied by a constant (k)
                    a = k * (m^2 - n^2), b = k * (2mn), c = k * (m^2 + n^2)
                
                x at this point the triple I'm after can be likely be discerned by finding each primitive triple where a + b + c <= 1000
                    then for each of those triples generated multiply their inputs by some value for K, and for those results find whichever hits 1000 exactly
                    provided that some primitive triple didn't hit the mark
                        so we need to find sets of three coprimes that when added together do not exceed 1000
                        need to dial back and study coprime solving better
                
                x again, coprime pairs only share a common divisor of 1, not necessarily prime themselves
                    coprime triples would be three integers that only share a common divisor of 1
                    coprime quads then would be 4, and so on for sets moving up the general rule is they lack common divisors
                
                
        */