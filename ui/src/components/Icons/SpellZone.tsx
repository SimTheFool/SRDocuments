import { Box } from "@radix-ui/themes";
import { TfiArrowsHorizontal } from "react-icons/tfi";
import { Spell } from "resources";
import { BaseIcon } from "./BaseIcon";

type SpellZoneProps = {
  zone: Spell["zone"];
};
export const SpellZone = ({ zone }: SpellZoneProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "text-top",
      }}
    >
      {zone && (
        <BaseIcon size={12} inline>
          <TfiArrowsHorizontal
            style={{
              color: "black",
            }}
          />
        </BaseIcon>
      )}
    </Box>
  );
};
