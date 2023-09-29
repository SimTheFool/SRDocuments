import { capitalize } from "@/utils/capitalize";
import { BaseItem } from "resources";
import { Card } from "../Card";
import { ParagraphStandard } from "../ParagraphStandard";
import { Price } from "../Price";
import { Space } from "../Space";
import { TitleMin } from "../TitleMin";
import { TextReplaced } from "../Text";
import { Box, Flex } from "@radix-ui/themes";
import { MasonryGrid } from "../MasonryGrid";
import styles from "./ItemCard.module.css";
import React from "react";
import { Slot } from "./Slot";
import { PiHandThin } from "react-icons/pi";
import { Hand } from "../Icons/Hand";

type ItemCardProps = {
  children?: {
    bottom?: React.ReactElement;
    inner?: React.ReactNode;
  };
  item: BaseItem;
  name: string;
};

export const ItemCard = ({ item, name, children }: ItemCardProps) => {
  const bottomChildren = React.Children.toArray(
    children?.bottom?.props.children
  ).filter((x) => x);

  const bottomChildrenWithSlots = [
    ...bottomChildren,
    ...(item.slots || []).map((slot) => {
      return (
        <Slot size={slot.size} concealment={slot.concealment}>
          {slot.name}
        </Slot>
      );
    }),
  ];

  const bottomItemNb = bottomChildrenWithSlots.length;

  return (
    <Box>
      <Flex
        className={bottomItemNb > 0 ? styles.noBorderBottom : ""}
        align={"start"}
      >
        <Card>
          <TitleMin
            inline
            title={
              <TextReplaced>
                {`${name.toUpperCase()} ${
                  item.quantity ? `x ${item.quantity}` : ""
                }`}
              </TextReplaced>
            }
            subtitle={item.manufacturer}
          />
          <TitleMin
            subtitle={
              <>
                <Price price={item.price} />
                <Space inline />
                {!item.legal ? "illégal" : item.licenced ? "licencié" : ""}
              </>
            }
          />
          <ParagraphStandard>
            {item.description && (
              <>
                <Space />
                <TextReplaced>{item.description}</TextReplaced>
              </>
            )}
            <Space />
            {children?.inner}
          </ParagraphStandard>
        </Card>
        <Box pl={"1"}>
          <Hand />
        </Box>
      </Flex>

      <MasonryGrid compact columns={1}>
        {bottomChildrenWithSlots.map((child, i) => (
          <Box className={i == 0 ? "" : styles.bottom}>{child}</Box>
        ))}
      </MasonryGrid>
    </Box>
  );
};
