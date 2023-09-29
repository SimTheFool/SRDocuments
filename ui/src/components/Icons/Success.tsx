import { Box } from "@radix-ui/themes";
import { ImDice } from "react-icons/im";
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
      <BaseIcon size={16}>
        <ImDice />
      </BaseIcon>
    </Box>
  );
};
