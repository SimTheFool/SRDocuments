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

type SpellActionProps = {
  name: string;
  action: SpellType;
};
export const SpellAction = ({
  name,
  action: { major, minor, descriptions, maintained, nature, range, type },
}: SpellActionProps) => {
  const spellSubtitle = [
    ...(maintained
      ? [
          <BaseIcon inline>
            <PiHourglassSimpleLowFill />
          </BaseIcon>,
        ]
      : []),
    nature,
    range,
  ];

  return (
    <Card title={type || null}>
      <Flex justify={"between"}>
        <Box>
          <TitleMin
            title={<TextReplaced>{capitalize(name)}</TextReplaced>}
            subtitle={spellSubtitle.map((c, i) => (
              <>
                {i !== 0 && " - "}
                {c}
              </>
            ))}
          />
          <Space />
          {Object.entries(descriptions || {}).map(([key, description]) => (
            <ParagraphStandard>
              {key !== "base" && (
                <>
                  <Space />
                  <TitleMin
                    title={
                      <TextReplaced>{`${capitalize(key)} :`}</TextReplaced>
                    }
                    inline
                  />
                </>
              )}
              <TextReplaced>{description}</TextReplaced>
            </ParagraphStandard>
          ))}
        </Box>
        <Box>
          {Array.from({ length: major }).map((_, i) => (
            <MajorAction key={i} />
          ))}
          {Array.from({ length: minor }).map((_, i) => (
            <MinorAction key={i} />
          ))}
        </Box>
      </Flex>
    </Card>
  );
};
