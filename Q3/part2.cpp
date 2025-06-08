/*
 * There are two new instructions you'll need to handle:
 *
 *     The do() instruction enables future mul instructions.
 *     The don't() instruction disables future mul instructions.
 *
 * Only the most recent do() or don't() instruction applies. At the beginning of the program, mul
 * instructions are enabled.
 *
 * For example:
 *
 * xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
 *
 * This corrupted memory is similar to the example from before, but this time the mul(5,5) and
 * mul(11,8) instructions are disabled because there is a don't() instruction before them. The other
 * mul instructions function normally, including the one at the end that gets re-enabled by a do()
 * instruction.
 *
 * This time, the sum of the results is 48 (2*4 + 8*5).
 */

#include <fstream>
#include <print>
#include <string>

int main(int argc, char *argv[])
{
    std::ifstream fileStrm("./input.txt");
    std::string strFile{std::istreambuf_iterator<char>{fileStrm}, std::istreambuf_iterator<char>{}};
    std::string::size_type curPos          = 0;
    const std::string::size_type sizeOfStr = strFile.size();
    int32_t totalSum                       = 0;

    bool mulEnabled = true;
    auto readNumber = [&](std::string::size_type &iIndex) -> std::string
    {
        std::string num;
        int maxDigits = 3;
        while (iIndex < sizeOfStr && std::isdigit(strFile[iIndex]) && maxDigits-- > 0)
            num += strFile[iIndex++];
        return num;
    };

    std::string_view const constexpr doIns   = "do()";
    std::string_view const constexpr dontIns = "don't()";
    std::string_view const constexpr mulIns  = "mul(";
    while (curPos < sizeOfStr)
    {
        if (strFile.compare(curPos, doIns.size(), doIns) == 0)
        {
            mulEnabled = true;
            curPos += doIns.size();
        }
        else if (strFile.compare(curPos, dontIns.size(), dontIns) == 0)
        {
            mulEnabled = false;
            curPos += dontIns.size();
        }
        else if (strFile.compare(curPos, mulIns.size(), mulIns) == 0)
        {
            curPos += mulIns.size();
            const auto strFirstNum = readNumber(curPos);
            if (strFirstNum.empty() || curPos >= sizeOfStr || strFile[curPos++] != ',')
                continue;

            const auto strSecondNum = readNumber(curPos);
            if (strSecondNum.empty() || curPos >= sizeOfStr || strFile[curPos++] != ')')
                continue;

            if (!mulEnabled)
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
        else
        {
            ++curPos;
        }
    }
    std::println("result: {}", totalSum);

    return 0;
}
