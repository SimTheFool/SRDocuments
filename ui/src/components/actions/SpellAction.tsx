import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { PiHourglassSimpleLowFill } from "react-icons/pi";
import { Spell as SpellType } from "resources";
import { Card } from "../Card";
import { MajorAction, MinorAction } from "../Icons/Actions";
import { BaseIcon } from "../Icons/BaseIcon";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { Maintained } from "../Icons/Maintained";
import { SpellNature } from "../Icons/SpellNature";
import { SpellDistance } from "../Icons/SpellDistance";

type SpellActionProps = {
  name: string;
  action: SpellType;
};
export const SpellAction = ({
  name,
  action: { major, minor, descriptions, maintained, nature, range, type },
}: SpellActionProps) => {
  const spellSubtitle = [
    range && <SpellDistance range={range} />,
    nature && <SpellNature nature={nature} />,
    maintained && <Maintained />,
  ];

  return (
    <Card title={type}>
      <Flex justify={"between"}>
        <Box>
          <TitleMin
            title={<TextReplaced>{capitalize(name)}</TextReplaced>}
            subtitle={spellSubtitle}
            inline
          />
          <Space />
          <Space />
          {Object.entries(descriptions || {}).map(([key, description]) => (
            <ParagraphStandard>
              {key === "base" ? (
                <TextReplaced>{description}</TextReplaced>
              ) : (
                <>
                  <Space />
                  <Space />
                  <TitleMin
                    title={
                      <TextReplaced>{`${capitalize(key)}: `}</TextReplaced>
                    }
                    inline
                  />
                  <Space />
                  <TextReplaced>{description}</TextReplaced>
                </>
              )}
            </ParagraphStandard>
          ))}
        </Box>
        <Box pt={"1"}>
          {Array.from({ length: major }).map((_, i) => (
            <MajorAction key={i} size={18} />
          ))}
          {Array.from({ length: minor }).map((_, i) => (
            <MinorAction key={i} size={18} />
          ))}
        </Box>
      </Flex>
    </Card>
  );
};
