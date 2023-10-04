import { Box } from "@radix-ui/themes";
import { PiHourglassSimpleLowFill } from "react-icons/pi";
import { BaseIcon } from "./BaseIcon";

type MaintainedProps = {};
export const Maintained = ({}: MaintainedProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "text-top",
      }}
    >
      <BaseIcon size={14} inline>
        <PiHourglassSimpleLowFill
          style={{
            color: "black",
          }}
        />
      </BaseIcon>
    </Box>
  );
};
