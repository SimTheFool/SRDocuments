import { Box } from "@radix-ui/themes";
import { RxTriangleUp, RxTriangleDown } from "react-icons/rx";

type AdvantageProps = {
  n: number;
};

export const Advantage = ({ n }: AdvantageProps) => {
  let negative = n < 0;
  let nAbs = Math.abs(n);
  return (
    <Box
      style={{
        display: "inline-block",
        position: "relative",
      }}
    >
      <Box
        style={{
          visibility: "hidden",
        }}
      >
        <RxTriangleUp />
      </Box>
      {Array.from({ length: nAbs }).map((_, i) => (
        <Box
          style={{
            position: "absolute",
            top: `${40 - i * 20}%`,
            transform: `translateY(${negative ? "-3px" : "0px"})`,
          }}
        >
          {negative ? <RxTriangleDown /> : <RxTriangleUp />}
        </Box>
      ))}
    </Box>
  );
};
