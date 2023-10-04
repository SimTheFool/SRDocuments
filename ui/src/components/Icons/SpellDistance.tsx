import { Box } from "@radix-ui/themes";
import { PiEyeLight, PiHandWaving } from "react-icons/pi";
import { TfiArrowsHorizontal } from "react-icons/tfi";
import { Spell } from "resources";
import { BaseIcon } from "./BaseIcon";

type SpellDistanceProps = {
  range: Spell["range"];
};
export const SpellDistance = ({ range }: SpellDistanceProps) => {
  return (
    <Box
      style={{
        display: "inline",
        verticalAlign: "text-top",
      }}
    >
      {range == "contact" && (
        <BaseIcon size={12} inline>
          <PiHandWaving
            style={{
              color: "black",
            }}
          />
        </BaseIcon>
      )}

      {(range == "LDV" || range == "LDV(Z)") && (
        <BaseIcon size={12} inline>
          <PiEyeLight
            style={{
              color: "black",
            }}
          />
        </BaseIcon>
      )}

      {range == "LDV(Z)" && (
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
