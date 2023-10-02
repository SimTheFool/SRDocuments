import { Box } from "@radix-ui/themes";
import { AiOutlineCheckSquare } from "react-icons/ai";
import { BaseIcon } from "./BaseIcon";

type SuccessProps = {};
export const Success = ({}: SuccessProps) => {
  return (
    <Box
      style={{
        display: "inline-block",
        transform: "translateY(1px)",
      }}
    >
      <BaseIcon size={18}>
        <AiOutlineCheckSquare
          style={{
            color: "black",
          }}
        />
      </BaseIcon>
    </Box>
  );
};
