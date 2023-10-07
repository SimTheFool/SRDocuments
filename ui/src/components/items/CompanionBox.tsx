import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { ReactNode } from "react";
import { Companion as CompanionType } from "resources";
import { Card } from "../Card";
import { MasonryGrid } from "../MasonryGrid";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { SimpleAction } from "../actions/SimpleAction";
import styles from "./ItemCard.module.css";
import { Slot } from "./Slot";
import { Effect } from "../actions/Effect";

type CompanionBoxProps = {
  name: string;
  type?: string;
  companion: CompanionType;
  children?: ReactNode;
  ergo?: boolean;
};

export const CompanionBox = ({
  name,
  type,
  companion,
  children,
  ergo = false,
}: CompanionBoxProps) => {
  const actions = Object.entries(companion.actions || {}).map(
    ([name, action]) => <SimpleAction name={name} action={action} key={name} />
  );

  const effects = Object.entries(companion.effects || {}).map(
    ([name, effect]) => <Effect effect={effect} key={name} simple />
  );

  const skills = companion.skills && (
    <Card style={{ backgroundColor: "var(--gray-6)" }}>
      <TitleMin title={<TextReplaced>{"Compétences"}</TextReplaced>} />
      <ParagraphStandard>{companion.skills.join(" - ")}</ParagraphStandard>
    </Card>
  );

  const invokSlot = (
    <Slot size="M">puissance - services - vie - maintient</Slot>
  );

  const bottomChildren = [
    ...effects,
    skills,
    ...actions,
    ...(ergo ? [] : [invokSlot]),
  ];

  return (
    <Box>
      <Flex
        className={
          bottomChildren.length > 0 && !ergo ? styles.noBorderBottom : ""
        }
      >
        <Card title={type}>
          <TitleMin title={<TextReplaced>{capitalize(name)}</TextReplaced>} />
          <ParagraphStandard>
            {companion.description && (
              <>
                <Space />
                <TextReplaced>{companion.description}</TextReplaced>
              </>
            )}
            <Space />
            {children}
            <Space />
          </ParagraphStandard>
        </Card>
      </Flex>
      <MasonryGrid compact columns={ergo ? 2 : 1}>
        {bottomChildren.map((child, i) => (
          <Box
            key={i}
            className={i == 0 || ergo ? "" : styles.bottom}
            p={ergo ? "1" : "0"}
          >
            {child}
          </Box>
        ))}
      </MasonryGrid>
      {invokSlot}
      <Space />
    </Box>
  );
};
