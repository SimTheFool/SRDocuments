import { Box } from "@radix-ui/themes";
import { AiOutlineCheckSquare } from "react-icons/ai";
import { BaseIcon } from "./BaseIcon";
import { CgDice5 } from "react-icons/cg";

type SuccessProps = {};
export const Success = ({}: SuccessProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "middle",
      }}
    >
      <BaseIcon size={14} inline>
        <CgDice5
          style={{
            color: "black",
          }}
        />
      </BaseIcon>
    </Box>
  );
};
