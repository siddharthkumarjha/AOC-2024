/*
 * It seems like the goal of the program is just to multiply some numbers. It does that with
 * instructions like mul(X,Y), where X and Y are each 1-3 digit numbers. For instance, mul(44,46)
 * multiplies 44 by 46 to get a result of 2024. Similarly, mul(123,4) would multiply 123 by 4.
 *
 * However, because the program's memory has been corrupted, there are also many invalid characters
 * that should be ignored, even if they look like part of a mul instruction. Sequences like mul(4*,
 * mul(6,9!, ?(12,34), or mul ( 2 , 4 ) do nothing.
 *
 * For example, consider the following section of corrupted memory:
 *
 * xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
 *
 * Only the four highlighted sections are real mul instructions. Adding up the result of each
 * instruction produces 161 (2*4 + 5*5 + 11*8 + 8*5).
 *
 * Scan the corrupted memory for uncorrupted mul instructions. What do you get if you add up all of
 * the results of the multiplications?
 */

#include <fstream>
#include <print>
#include <string>

int main(int argc, char *argv[])
{
    std::ifstream fileStrm("./example.txt");
    std::string strFile{std::istreambuf_iterator<char>{fileStrm}, std::istreambuf_iterator<char>{}};
    std::string::size_type mulInsPos       = 0;
    const std::string::size_type sizeOfStr = strFile.size();
    int32_t totalSum = 0;

    while ((mulInsPos = strFile.find("mul(", mulInsPos)) != std::string::npos)
    {
        mulInsPos += 4;

        auto readNumber = [&](std::string::size_type &iIndex) -> std::string
        {
            std::string num;
            int maxDigits = 3;
            while (iIndex < sizeOfStr && std::isdigit(strFile[iIndex]) && maxDigits-- > 0)
                num += strFile[iIndex++];
            return num;
        };

        const auto strFirstNum = readNumber(mulInsPos);
        if (strFirstNum.empty() || mulInsPos >= sizeOfStr || strFile[mulInsPos++] != ',')
            continue;

        const auto strSecondNum = readNumber(mulInsPos);
        if (strSecondNum.empty() || mulInsPos >= sizeOfStr || strFile[mulInsPos++] != ')')
            continue;

        try
        {
            const int32_t intFirstNum  = std::stoi(strFirstNum);
            const int32_t intSecondNum = std::stoi(strSecondNum);
            const int32_t mulOpResult  = intFirstNum * intSecondNum;
            totalSum += mulOpResult;
            std::println("{} * {} = {}", strFirstNum, strSecondNum, mulOpResult);
        }
        catch (std::exception const &excuse)
        {
            std::print(stderr, "{}\n", excuse.what());
        }
    }
    std::println("result: {}", totalSum);

    return 0;
}
