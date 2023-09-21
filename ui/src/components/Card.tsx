import {
  Box,
  Container,
  Heading,
  Inset,
  Card as RadCard,
  Text,
} from "@radix-ui/themes";
import styles from "./Card.module.css";
import React from "react";

type CardProps = {
  children?: React.ReactNode;
  title?: React.ReactNode;
  note?: React.ReactNode;
};

export const Card = ({ title, children, note }: CardProps) => {
  return (
    <RadCard className={styles.card}>
      <Box className={styles.cardTitle} px={"1"} asChild>
        <Heading size={"1"} as={"h3"} weight={"light"}>
          {title}
        </Heading>
      </Box>
      <Box className={styles.cardNote} px={"1"} asChild>
        <Heading size={"1"} as={"h3"} weight={"light"}>
          {note}
        </Heading>
      </Box>
      {children}
    </RadCard>
  );
};
