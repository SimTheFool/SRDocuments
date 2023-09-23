import { Box } from "@radix-ui/themes";
import { BsFillDiamondFill } from "react-icons/bs";
import { BaseIcon } from "./BaseIcon";

type AdvantageProps = {
  n: number;
};

export const Advantage = ({ n }: AdvantageProps) => {
  return (
    <Box
      style={{
        display: "inline-block",
        position: "relative",
      }}
    >
      <BaseIcon size={18}>
        <BsFillDiamondFill />
      </BaseIcon>
      <Box
        style={{
          position: "absolute",
          color: "white",
          top: "13%",
          left: "20%",
        }}
      >
        {n}
      </Box>
    </Box>
  );
};
